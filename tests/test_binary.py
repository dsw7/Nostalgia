import os
import subprocess
import pytest

PATH_BINARY = os.path.join(os.getcwd(), 'target/debug/nos')

@pytest.mark.parametrize('args', [PATH_BINARY, f'{PATH_BINARY} --period=abc'])
def test_invalid_args(args: str) -> None:
    with pytest.raises(subprocess.CalledProcessError):
        subprocess.check_output(args.split(), stderr=subprocess.DEVNULL)
