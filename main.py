import pandas as pd

df = pd.read_csv("cpu_instruction.csv")

for _, r in df.iterrows():
    name = r.get('name')
    operation = r.get('operation')
    addressingMode = r.get('addressingMode')
    cycle = r.get('cycles')
    print(f"Instruction::new(\"{name}\".to_string(),Some(Cpu::{operation}),Some(Cpu::{addressingMode}),{cycle}),")

