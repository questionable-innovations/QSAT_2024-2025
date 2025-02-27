import os
from platformio import util

def pre_build_patching(env):
    # Get PlatformIO home directory using the API
    pio_home = util.get_home_dir()
    
    # Path to energia.py
    energia_path = os.path.join(pio_home, 
                               'platforms', 'timsp430', 'builder', 
                               'frameworks', 'energia.py')
    
    if not os.path.exists(energia_path):
        print(f"Warning: Could not find energia.py at {energia_path}")
        return
    
    try:
        with open(energia_path, 'r') as file:
            content = file.read()
        
        # Replace the problematic line
        fixed_content = content.replace(
            'join(platform.get_package_dir("toolchain-timsp430"), "msp430", "include")',
            'join(platform.get_package_dir("toolchain-timsp430") or os.path.join(util.get_home_dir(), "packages/toolchain-timsp430"), "msp430", "include")'
        )
        
        with open(energia_path, 'w') as file:
            file.write(fixed_content)
        print("Successfully patched energia.py")
    except Exception as e:
        print(f"Error patching energia.py: {e}")