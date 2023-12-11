import os

def rename_folders(directory_path, old_name, new_name):
    try:
        for folder_name in os.listdir(directory_path):
            if len(folder_name) == 1:
                os.rename(os.path.join(directory_path, folder_name), os.path.join(directory_path, "0".join(folder_name)))
    except OSError as e:
        print(f"Error: {e}")

# Provide the directory path, old folder name, and new folder name
directory = './'
old_folder_name = 'day'
new_folder_name = ''

rename_folders(directory, old_folder_name, new_folder_name)

