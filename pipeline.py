import subprocess
import sys
import os
from openai import OpenAI

def run_build_command(command):
    try:
        # Run the cargo command
        result = subprocess.run(
            command,           # The command to run as a list
            stdout=subprocess.PIPE,   # Capture the output
            stderr=subprocess.PIPE,   # Capture errors
            text=True,                # Decode bytes to str
            check=True                # Raise exception if command fails
        )
        return result.stdout
    except subprocess.CalledProcessError as e:
        # Handle errors if the command fails
        print(f"Error occurred: {e.stderr}")
        return None


# Command Usage: python3 pipeline.py prompt.txt source_file

# read in prompt file
with open(sys.argv[1], 'r', encoding='utf-8') as file:
    prompt = file.read()
# read in C source code files
with open(sys.argv[2] + ".c", 'r', encoding='utf-8') as file:
    program_text = file.read()

# # Send to OpenAI, not the most expensive model!
# client = OpenAI()

# completion = client.chat.completions.create(
#     model="gpt-4o-mini",
#     messages=[
#         {"role": "system", "content": "You are a helpful assistant."},
#         {
#             "role": "user",
#             "content": prompt + program_text
#         }
#     ]
# )

# rust_translation = completion.choices[0].message.replace("\\n", "\n")

with open("output.rs", 'r', encoding='utf-8') as file:
    rust_translation = file.read();

# append translated rust code to corresponding files
with open("rust-libm/src/math/" + sys.argv[2] + ".rs", 'a', encoding='utf-8') as file:
    file.write(rust_translation)

print(run_build_command(["cargo", "build", "--manifest-path", "rust-libm/Cargo.toml"]))
