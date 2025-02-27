import os
from os.path import isdir, join, exists
from SCons.Script import DefaultEnvironment, Import
print("Patching toolchain-timsp430 include path for ELF toolchain...")
Import("env")
platform = env.PioPlatform()

# Function to find include directory in the new ELF-based toolchain structure
def find_toolchain_dir():
    print(platform.get_installed_packages())
    try:
        default_path = platform.get_package_dir("toolchain-timsp430")
        print(default_path)
        if default_path is not None and isdir(join(default_path, "msp430-elf", "include")):
            print("Found in default path:", default_path)
            return join(default_path, "msp430-elf", "include")
    except Exception as e:
        print(f"Error checking default path: {e}")

    # Check in .pio/packages
    try:
        packages_path = join("~",".platformio", "packages", "toolchain-timsp430")
        print(packages_path, isdir(join(packages_path, "msp430-elf", "include")))

        if exists(join(packages_path, "msp430-elf", "include")):
            print("Found in packages path:", packages_path)
            return join(packages_path, "msp430-elf", "include")
    except Exception as e:
        print(f"Error checking packages path: {e}")

# Patch energia.py's toolchain path finder
original_get_package_dir = platform.get_package_dir
def custom_get_package_dir(package_name):
    result = original_get_package_dir(package_name)
    if package_name == "toolchain-timsp430":
        print(f"Original toolchain path: {result}")
    return result

platform.get_package_dir = custom_get_package_dir

# Override compiler settings to use correct paths
env.Replace(
    CC=join("$PIOHOME_DIR", "packages", "toolchain-timsp430", "bin", "msp430-elf-gcc"),
    CXX=join("$PIOHOME_DIR", "packages", "toolchain-timsp430", "bin", "msp430-elf-g++"),
    AR=join("$PIOHOME_DIR", "packages", "toolchain-timsp430", "bin", "msp430-elf-ar"),
    RANLIB=join("$PIOHOME_DIR", "packages", "toolchain-timsp430", "bin", "msp430-elf-ranlib"),
    SIZETOOL=join("$PIOHOME_DIR", "packages", "toolchain-timsp430", "bin", "msp430-elf-size"),
    OBJCOPY=join("$PIOHOME_DIR", "packages", "toolchain-timsp430", "bin", "msp430-elf-objcopy"),
)

# Add the include path explicitly
toolchain_include = find_toolchain_dir()
if toolchain_include:
    print(f"Adding include path: {toolchain_include}")
    env.Append(CPPPATH=[toolchain_include])