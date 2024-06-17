# Simple python script to extract all passwords and discord token to one master txt file
# How to use? Just place script in the same directory as all zip files and run it, it will create two txt files

import os
import zipfile
import re

def remove_blank_lines(file_path):
    with open(file_path, 'r', encoding='utf-8') as file:
        lines = file.readlines()
    with open(file_path, 'w', encoding='utf-8') as file:
        lines = [line for line in lines if line.strip() != '']
        file.writelines(lines)

def extract_folders():
    dir_path = os.path.dirname(os.path.realpath(__file__))

    # Create a file to store all passwords
    passwords_file_path = os.path.join(dir_path, 'all_passwords.txt')
    passwords_file = open(passwords_file_path, 'w', encoding='utf-8')
    # Create a file to store all tokens
    tokens_file_path = os.path.join(dir_path, 'all_tokens.txt')
    tokens_file = open(tokens_file_path, 'w', encoding='utf-8')

    for item in os.listdir(dir_path):
        if item.endswith('.zip'):
            try:
                with zipfile.ZipFile(item, 'r') as zip_ref:
                    for file in zip_ref.namelist():
                        if file.startswith('Passwords/'):
                            # Extract all files from the Passwords folder and append their contents to all_passwords.txt
                            password = zip_ref.read(file).decode('utf-8')
                            # Remove lines that do not match the expression
                            password = '\n'.join([line for line in password.split('\n') if re.match(r'h.*:.*:', line)])
                            passwords_file.write(password + '\n')
                        elif file.startswith('Discord/') and file.endswith('tokens.txt'):
                            # Extract tokens.txt from the Discord folder and append its contents to all_tokens.txt
                            token_data = zip_ref.read(file).decode('utf-8')
                            # Remove 'Token: ' from each token before writing it to the file
                            tokens = [token.replace('Token: ', '') for token in token_data.split('\n')]
                            # Only save tokens that are longer than 40 characters
                            for token in tokens:
                                if len(token) > 40:
                                    tokens_file.write(token + '\n')
            except zipfile.BadZipFile:
                print(f"Skipping file {item} because it's not a valid zip file.")

    passwords_file.close()
    tokens_file.close()

    # Remove blank lines from the generated files
    remove_blank_lines(passwords_file_path)
    remove_blank_lines(tokens_file_path)
	
    # Count the number of lines (passwords/tokens) in the files
    password_count = sum(1 for line in open(passwords_file_path, 'r', encoding='utf-8'))
    token_count = sum(1 for line in open(tokens_file_path, 'r', encoding='utf-8'))

    print(f"Passwords: {password_count}")
    print(f"Tokens: {token_count}")

if __name__ == "__main__":
    extract_folders()
