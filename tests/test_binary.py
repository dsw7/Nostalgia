import typing
import os
import subprocess
import pytest

PATH_BINARY = os.path.join(os.getcwd(), 'target/debug/nos')

if not os.path.exists(PATH_BINARY):
    pytest.exit(f'{PATH_BINARY} does not exist!')

TEST_CASES_1 = [
    [PATH_BINARY],
    [PATH_BINARY, '--period=abc'],
    [PATH_BINARY, '-p=abc'],
    [PATH_BINARY, '--period=5', '--unit=foo']
]

@pytest.mark.parametrize('command', TEST_CASES_1, ids=[' '.join(s) for s in TEST_CASES_1])
def test_invalid_args(command: typing.List[str]) -> None:
    with pytest.raises(subprocess.CalledProcessError):
        subprocess.check_output(command, stderr=subprocess.DEVNULL)
