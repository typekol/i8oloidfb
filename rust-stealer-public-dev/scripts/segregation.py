# Simple python script to extract all ZIP files with Wallets + FileZilla servers
# How to use? Just place script in the same directory as all ZIP files and run it, it will create new folder with all ZIP files

import os
import shutil
import zipfile

# Define the directory where the zip files are located
zip_dir = os.getcwd()

# Create a new directory for the copied zip files
new_dir = 'Files'
os.makedirs(new_dir, exist_ok=True)

# Initialize counters
wallet_count = 0
ftp_count = 0

# Iterate over all files in the directory
for filename in os.listdir(zip_dir):
    if filename.endswith('.zip'):
        # Open the zip file
        with zipfile.ZipFile(os.path.join(zip_dir, filename), 'r') as zip_ref:
            # Check if 'Wallets' or 'FTP' directory exists in the zip file
            if any('Wallets' in name for name in zip_ref.namelist()):
                wallet_count += 1
                shutil.copy(os.path.join(zip_dir, filename), new_dir)
            if any('FileZilla' in name for name in zip_ref.namelist()):
                ftp_count += 1
                shutil.copy(os.path.join(zip_dir, filename), new_dir)

print(f'Wallets: {wallet_count}\nServers: {ftp_count}')
