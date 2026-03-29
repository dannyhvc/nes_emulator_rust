use iced::{
    widget::{
        button, column, container, horizontal_rule, pane_grid, row, scrollable,
        text, text_input, Column, Row, Space,
    },
    Alignment, Color, Element, Font, Length, Size, Task, Theme,
};
/// NES / MOS 6502 Debugger — iced 0.13.1
use std::collections::BTreeMap;

use components::{dh_bus::BUS, dh_cpu::CPU};

// Colour palette

const BG: Color = Color {
    r: 0.10,
    g: 0.10,
    b: 0.14,
    a: 1.0,
};
const PANEL: Color = Color {
    r: 0.14,
    g: 0.14,
    b: 0.20,
    a: 1.0,
};
const PANEL_BORDER: Color = Color {
    r: 0.28,
    g: 0.28,
    b: 0.40,
    a: 1.0,
};
const ACCENT: Color = Color {
    r: 0.45,
    g: 0.35,
    b: 0.85,
    a: 1.0,
};
const TEXT_DIM: Color = Color {
    r: 0.55,
    g: 0.55,
    b: 0.65,
    a: 1.0,
};
const TEXT_BRIGHT: Color = Color {
    r: 0.88,
    g: 0.88,
    b: 0.95,
    a: 1.0,
};
const HIGHLIGHT: Color = Color {
    r: 1.00,
    g: 0.88,
    b: 0.30,
    a: 1.0,
};
const HIGHLIGHT_BG: Color = Color {
    r: 0.25,
    g: 0.22,
    b: 0.10,
    a: 1.0,
};
const FLAG_ON: Color = Color {
    r: 0.35,
    g: 0.95,
    b: 0.55,
    a: 1.0,
};
const FLAG_OFF: Color = Color {
    r: 0.35,
    g: 0.35,
    b: 0.45,
    a: 1.0,
};
const PC_BYTE: Color = Color {
    r: 1.00,
    g: 0.65,
    b: 0.20,
    a: 1.0,
};
const INPUT_ERR: Color = Color {
    r: 0.95,
    g: 0.30,
    b: 0.30,
    a: 1.0,
};

// Demo program
//
// Fibonacci loop in flat writable RAM at $0200 (safe on any NES bus layout).
//
//   $0200  LDA #$01   ; seed A = 1
//   $0202  STA $00    ; mem[0] = 1
//   $0204  LDA #$01
//   $0206  STA $01    ; mem[1] = 1
//   $0208  LDA $00    ; A = mem[n-2]   ← loop top
//   $020A  ADC $01    ; A += mem[n-1]
//   $020C  STA $02    ; mem[2] = result
//   $020E  LDA $01    ; rotate: mem[0] ← old mem[1]
//   $0210  STA $00
//   $0212  LDA $02    ;         mem[1] ← result
//   $0214  STA $01
//   $0216  JMP $0208  ; loop
//
const PROG_START: u16 = 0x0200;
const PROG_END: u16 = 0x0218;

const PROG: &[(u16, u8)] = &[
    (0x0200, 0xA9),
    (0x0201, 0x01), // LDA #$01
    (0x0202, 0x85),
    (0x0203, 0x00), // STA $00
    (0x0204, 0xA9),
    (0x0205, 0x01), // LDA #$01
    (0x0206, 0x85),
    (0x0207, 0x01), // STA $01
    (0x0208, 0xA5),
    (0x0209, 0x00), // LDA $00
    (0x020A, 0x65),
    (0x020B, 0x01), // ADC $01
    (0x020C, 0x85),
    (0x020D, 0x02), // STA $02
    (0x020E, 0xA5),
    (0x020F, 0x01), // LDA $01
    (0x0210, 0x85),
    (0x0211, 0x00), // STA $00
    (0x0212, 0xA5),
    (0x0213, 0x02), // LDA $02
    (0x0214, 0x85),
    (0x0215, 0x01), // STA $01
    (0x0216, 0x4C),
    (0x0217, 0x08),
    (0x0218, 0x02), // JMP $0208
];

// App state
// Add a quick identifier for which pane is which
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneKind {
    Left,
    Right,
}
pub struct Debugger {
    cpu: CPU,
    bus: BUS,
    disasm: BTreeMap<u16, String>,
    running: bool,
    ram_page: u8,
    ram_page_input: String,
    ram_page_input_err: bool,
    follow_pc: bool,
    run_burst: u32,
    panes: pane_grid::State<PaneKind>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Step,
    Run,
    Stop,
    Reset,
    RamPagePrev,
    RamPageNext,
    RamPageInputChanged(String),
    RamPageInputSubmit,
    ToggleFollowPc,
    Resized(pane_grid::ResizeEvent),
}

// Entry point
pub fn run() -> iced::Result {
    iced::application(
        "NES Debugger — MOS 6502",
        Debugger::update,
        Debugger::view,
    )
    .theme(|_| Theme::Dark)
    .window(iced::window::Settings {
        size: Size::new(1200.0, 820.0),
        min_size: Some(Size::new(900.0, 600.0)),
        ..Default::default()
    })
    .default_font(Font::DEFAULT)
    .run_with(Debugger::new)
}

// Init & update
impl Debugger {
    fn new() -> (Self, Task<Message>) {
        let mut cpu = CPU::new();
        let mut bus = BUS::new();

        for &(addr, val) in PROG {
            bus.write(addr, val);
        }

        CPU::reset(&mut cpu, &bus);
        for _ in 0..8 {
            CPU::clock(&mut cpu, &mut bus);
        }
        // Force PC to program start — the NES bus maps $FFFC/$FFFD to
        // cartridge ROM, so we can't rely on the reset vector.
        cpu.set_pc(PROG_START);

        let disasm: BTreeMap<u16, String> =
            CPU::disassemble(&mut bus, PROG_START, PROG_END)
                .into_iter()
                .map(|(k, v)| (k, v.to_uppercase()))
                .collect();

        let initial_page = (PROG_START >> 8) as u8;

        // Initialize the pane grid
        let (mut panes, left_pane) = pane_grid::State::new(PaneKind::Left);

        // Split vertically to create the right pane
        if let Some((_right_pane, split)) =
            panes.split(pane_grid::Axis::Vertical, left_pane, PaneKind::Right)
        {
            // Set initial boundary: 40% left, 60% right
            panes.resize(split, 0.40);
        }

        (
            Self {
                cpu,
                bus,
                disasm,
                running: false,
                ram_page: initial_page,
                ram_page_input: format!("{:02X}", initial_page),
                ram_page_input_err: false,
                follow_pc: true,
                run_burst: 500,
                panes, // Add to struct
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Step => {
                while !self.cpu.complete() {
                    CPU::clock(&mut self.cpu, &mut self.bus);
                }
                loop {
                    CPU::clock(&mut self.cpu, &mut self.bus);
                    if self.cpu.complete() {
                        break;
                    }
                }
                self.sync_page();
            }
            Message::Run => {
                self.running = true;
                for _ in 0..self.run_burst {
                    CPU::clock(&mut self.cpu, &mut self.bus);
                }
                self.sync_page();
            }
            Message::Stop => {
                self.running = false;
            }
            Message::Reset => {
                self.running = false;
                CPU::reset(&mut self.cpu, &self.bus);
                for _ in 0..8 {
                    CPU::clock(&mut self.cpu, &mut self.bus);
                }
                self.cpu.set_pc(PROG_START);
                self.sync_page();
            }
            Message::RamPagePrev => {
                self.ram_page = self.ram_page.saturating_sub(1);
                self.ram_page_input = format!("{:02X}", self.ram_page);
                self.ram_page_input_err = false;
            }
            Message::RamPageNext => {
                if self.ram_page < 255 {
                    self.ram_page += 1;
                }
                self.ram_page_input = format!("{:02X}", self.ram_page);
                self.ram_page_input_err = false;
            }
            Message::RamPageInputChanged(s) => {
                if s.len() <= 3 {
                    self.ram_page_input_err =
                        !s.is_empty() && parse_page(&s).is_none();
                    self.ram_page_input = s;
                }
            }
            Message::RamPageInputSubmit => {
                match parse_page(&self.ram_page_input) {
                    Some(p) => {
                        self.ram_page = p;
                        self.ram_page_input_err = false;
                    }
                    None => {
                        self.ram_page_input_err = true;
                    }
                }
            }
            Message::ToggleFollowPc => {
                self.follow_pc = !self.follow_pc;
                if self.follow_pc {
                    self.sync_page();
                }
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
        }
        Task::none()
    }

    fn sync_page(&mut self) {
        if self.follow_pc {
            let page = (self.cpu.pc() >> 8) as u8;
            self.ram_page = page;
            self.ram_page_input = format!("{:02X}", page);
            self.ram_page_input_err = false;
        }
    }
}

// View
impl Debugger {
    fn view(&self) -> Element<'_, Message> {
        let grid = pane_grid(&self.panes, |_pane, state, _is_maximized| {
            // Match the stored PaneKind to its content
            let content: Element<Message> = match state {
                PaneKind::Left => column![
                    self.view_registers(),
                    self.view_flags(),
                    self.view_ram(),
                ]
                .spacing(10)
                .height(Length::Fill)
                .into(),

                PaneKind::Right => column![self.view_disassembly()]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into(),
            };

            pane_grid::Content::new(content)
        })
        .on_resize(10.0, Message::Resized) // 10px invisible grab handle
        .spacing(14) // Gap between panes (replaces row spacing)
        .width(Length::Fill)
        .height(Length::Fill);

        let root = column![self.view_top_bar(), horizontal_rule(1), grid]
            .spacing(10)
            .padding(14);

        container(root)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_: &Theme| iced::widget::container::Style {
                background: Some(iced::Background::Color(BG)),
                ..Default::default()
            })
            .into()
    }

    // Top bar
    fn view_top_bar(&self) -> Element<Message> {
        let title = text("MOS 6502 Debugger").size(20).color(ACCENT);

        let btn = |label: &'static str, msg: Message| {
            button(text(label).size(13).color(TEXT_BRIGHT))
                .on_press(msg)
                .padding([6, 16])
        };

        let btn_run_stop = if self.running {
            btn("  Stop  ", Message::Stop)
        } else {
            btn("  Run   ", Message::Run)
        };

        let clock_label = text(format!(
            "clk: {}   ins: {}",
            self.cpu.clock_count(),
            self.cpu.clock_count() / 3,
        ))
        .size(12)
        .color(TEXT_DIM);

        row![
            title,
            Space::with_width(24),
            btn("  Step  ", Message::Step),
            btn_run_stop,
            btn(" Reset  ", Message::Reset),
            Space::with_width(Length::Fill),
            clock_label,
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .into()
    }

    // Registers
    fn view_registers(&self) -> Element<Message> {
        let pc = self.cpu.pc();
        let sp = self.cpu.sp();
        let a = self.cpu.a();
        let x = self.cpu.x();
        let y = self.cpu.y();

        let reg = |name: &'static str,
                   hex: String,
                   dec: String|
         -> Element<Message> {
            row![
                text(name)
                    .size(14)
                    .font(Font::MONOSPACE)
                    .color(TEXT_DIM)
                    .width(Length::Fixed(32.0)),
                text(hex)
                    .size(15)
                    .font(Font::MONOSPACE)
                    .color(HIGHLIGHT)
                    .width(Length::Fixed(76.0)),
                text(dec).size(13).font(Font::MONOSPACE).color(TEXT_DIM),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .into()
        };

        let rows = column![
            text("Registers").size(14).color(ACCENT),
            horizontal_rule(1),
            reg("PC", format!("${:04X}", pc), format!("({})", pc)),
            reg(
                "SP",
                format!("${:02X}", sp),
                format!("(stack: ${:04X})", 0x0100u16 + sp as u16)
            ),
            reg("A", format!("${:02X}", a), format!("({:3})", a)),
            reg("X", format!("${:02X}", x), format!("({:3})", x)),
            reg("Y", format!("${:02X}", y), format!("({:3})", y)),
        ]
        .spacing(6)
        .padding(12);

        container(rows)
            .width(Length::Fill)
            .style(panel_style)
            .into()
    }

    // Flags
    fn view_flags(&self) -> Element<Message> {
        let status = self.cpu.status();

        let flag = |name: &'static str, mask: u8| -> Element<Message> {
            let on = status & mask != 0;
            let color = if on { FLAG_ON } else { FLAG_OFF };
            column![
                text(name).size(11).color(color),
                text(if on { "1" } else { "0" })
                    .size(16)
                    .font(Font::MONOSPACE)
                    .color(color),
            ]
            .align_x(Alignment::Center)
            .spacing(3)
            .width(Length::Fixed(36.0))
            .into()
        };

        let inner = column![
            text("Flags").size(14).color(ACCENT),
            horizontal_rule(1),
            row![
                flag("N", 0x80),
                flag("V", 0x40),
                flag("-", 0x20),
                flag("B", 0x10),
                flag("D", 0x08),
                flag("I", 0x04),
                flag("Z", 0x02),
                flag("C", 0x01),
            ]
            .spacing(4)
            .align_y(Alignment::End),
            text(format!("${:02X}  0b{:08b}", status, status))
                .size(12)
                .font(Font::MONOSPACE)
                .color(TEXT_DIM),
        ]
        .spacing(8)
        .padding(12);

        container(inner)
            .width(Length::Fill)
            .style(panel_style)
            .into()
    }

    // RAM hex view
    fn view_ram(&self) -> Element<Message> {
        let page = self.ram_page as u16;
        let base = page * 256;
        let pc = self.cpu.pc();

        // Column header
        let header =
            text("        00 01 02 03 04 05 06 07   08 09 0A 0B 0C 0D 0E 0F")
                .size(12)
                .font(Font::MONOSPACE)
                .color(TEXT_DIM);

        let mut hex_rows: Vec<Element<Message>> = vec![header.into()];

        for r in 0..16u16 {
            let row_base = base + r * 16;

            let addr_lbl = text(format!("${:04X}: ", row_base))
                .size(12)
                .font(Font::MONOSPACE)
                .color(TEXT_DIM)
                .width(Length::Fixed(58.0));

            let mut cells: Vec<Element<Message>> = vec![addr_lbl.into()];

            for c in 0..16u16 {
                let byte_addr = row_base + c;
                let byte = self.bus.read(byte_addr, true);
                let color = if byte_addr == pc {
                    PC_BYTE
                } else {
                    TEXT_BRIGHT
                };
                // Extra gap between the two 8-byte halves
                let sep = if c == 8 { "  " } else { "" };
                cells.push(
                    text(format!("{}{:02X}", sep, byte))
                        .size(12)
                        .font(Font::MONOSPACE)
                        .color(color)
                        .into(),
                );
            }

            hex_rows.push(
                Row::from_iter(cells)
                    .spacing(5)
                    .align_y(Alignment::Center)
                    .into(),
            );
        }

        // Nav bar
        let border_color = if self.ram_page_input_err {
            INPUT_ERR
        } else {
            PANEL_BORDER
        };

        let page_input = text_input("hex / dec", &self.ram_page_input)
            .on_input(Message::RamPageInputChanged)
            .on_submit(Message::RamPageInputSubmit)
            .font(Font::MONOSPACE)
            .size(13)
            .width(Length::Fixed(54.0))
            .style(move |theme: &Theme, status| {
                let mut s = text_input::default(theme, status);
                s.border.color = border_color;
                s.border.width = 1.5;
                s.border.radius = 4.0.into();
                s
            });

        // Hint below the input, not inline — prevents layout overflow/popover.
        let hint_text = if self.ram_page_input_err {
            text("invalid").size(10).color(INPUT_ERR)
        } else {
            text(format!("${:04X}–${:04X}", base, base + 255))
                .size(10)
                .color(TEXT_DIM)
        };

        let follow_color = if self.follow_pc { HIGHLIGHT } else { TEXT_DIM };
        let follow_label = if self.follow_pc {
            "Follow PC *"
        } else {
            "Follow PC"
        };
        let follow_btn =
            button(text(follow_label).size(11).color(follow_color))
                .on_press(Message::ToggleFollowPc)
                .padding([3, 8]);

        // Page input + hint stacked vertically so hint never overflows
        let page_field: Element<Message> = column![
            row![text("Page $").size(12).color(TEXT_DIM), page_input,]
                .spacing(4)
                .align_y(Alignment::Center),
            hint_text,
        ]
        .spacing(2)
        .into();

        let nav = row![
            button(text("<").size(12))
                .on_press(Message::RamPagePrev)
                .padding([4, 10]),
            page_field,
            Space::with_width(Length::Fill),
            follow_btn,
            button(text(">").size(12))
                .on_press(Message::RamPageNext)
                .padding([4, 10]),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let inner = column![
            row![
                text("RAM").size(14).color(ACCENT),
                Space::with_width(Length::Fill),
                nav,
            ]
            .align_y(Alignment::Center),
            horizontal_rule(1),
            // Fill the remaining height of the panel — no fixed pixel cap.
            scrollable(Column::from_iter(hex_rows).spacing(3).padding([0, 4]))
                .height(Length::Fill),
        ]
        .spacing(8)
        .padding(12)
        .height(Length::Fill); // panel stretches to bottom of left column

        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(panel_style)
            .into()
    }

    // Disassembly
    fn view_disassembly(&self) -> Element<Message> {
        let pc = self.cpu.pc();

        let entries = self.disasm.iter().map(|(&addr, instr)| {
            let is_pc = addr == pc;

            let row_inner = row![
                text(if is_pc { ">" } else { " " })
                    .size(13)
                    .color(HIGHLIGHT)
                    .width(Length::Fixed(18.0)),
                text(format!("${:04X}", addr))
                    .size(13)
                    .font(Font::MONOSPACE)
                    .color(TEXT_DIM)
                    .width(Length::Fixed(62.0)),
                text(instr).size(13).font(Font::MONOSPACE).color(if is_pc {
                    HIGHLIGHT
                } else {
                    TEXT_BRIGHT
                }),
            ]
            .spacing(6)
            .align_y(Alignment::Center)
            .padding([3, 8]);

            let c: Element<Message> = if is_pc {
                container(row_inner)
                    .style(|_: &Theme| iced::widget::container::Style {
                        background: Some(iced::Background::Color(HIGHLIGHT_BG)),
                        border: iced::Border {
                            radius: 4.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .width(Length::Fill)
                    .into()
            } else {
                container(row_inner).width(Length::Fill).into()
            };
            c
        });

        let inner = column![
            text("Disassembly").size(14).color(ACCENT),
            horizontal_rule(1),
            scrollable(
                Column::from_iter(entries).spacing(2).width(Length::Fill)
            )
            .height(Length::Fill),
        ]
        .spacing(8)
        .padding(12)
        .height(Length::Fill);

        container(inner)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(panel_style)
            .into()
    }
}

// Helpers
/// Parse a RAM page (0–255).
/// - Has any A-F letter → hex    ("C0", "ff")
/// - Pure digits ≤ 2 chars → hex ("10" = page 16)
/// - Pure digits = 3 chars → dec ("192", "255")
fn parse_page(s: &str) -> Option<u8> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let has_alpha = s.chars().any(|c| c.is_ascii_alphabetic());
    let all_hex = s.chars().all(|c| c.is_ascii_hexdigit());
    if !all_hex {
        return None;
    }
    if has_alpha || s.len() <= 2 {
        u8::from_str_radix(s, 16).ok()
    } else {
        s.parse::<u8>().ok()
    }
}

fn panel_style(_: &Theme) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(iced::Background::Color(PANEL)),
        border: iced::Border {
            color: PANEL_BORDER,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}
