"""Select a native Bash for script checks; Windows' System32 bash is WSL."""
import os
from pathlib import Path
import shutil


def bash_executable(platform=None):
    platform = platform or os.name
    if platform == 'nt':
        git = shutil.which('git')
        candidates = []
        if git:
            git_root = Path(git).resolve().parent.parent
            candidates.extend([git_root / 'bin/bash.exe', git_root / 'usr/bin/bash.exe'])
        program_files = os.environ.get('ProgramFiles')
        if program_files:
            candidates.append(Path(program_files) / 'Git/bin/bash.exe')
        for candidate in candidates:
            if candidate.is_file():
                return str(candidate)
        raise RuntimeError('Git Bash is required for Windows checks; System32/bash.exe would start WSL')
    bash = shutil.which('bash')
    if not bash:
        raise RuntimeError('Bash is required for shell examples and release checks')
    return bash
