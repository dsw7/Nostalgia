import typing
import os
import subprocess
import pytest

PATH_BINARY = os.path.join(os.getcwd(), 'target/debug/nos')
PATH_RESULTS_FILE = '/tmp/nos.txt'
R1 = 99.5 * 1000
R2 = 21.5 * 1000

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

def test_invalid_period() -> None:

    try:
        subprocess.check_output([PATH_BINARY, '--period=0'], stderr=subprocess.PIPE)
    except subprocess.CalledProcessError as e:
        assert e.stderr.decode() == 'Period is too small! Minimum period is 0.0001 seconds\n'
    else:
        pytest.fail('Did not raise CalledProcessError')

def compute_expected_capacitance(period: float) -> typing.Tuple[float, float, float, float]:

    frequency = 1 / period
    capacitance = 1.44 / (frequency * (R1 + 2 * R2))
    return frequency, capacitance, capacitance * 10 ** 6, capacitance * 10 ** 9

TEST_CASES_2 = [
    [PATH_BINARY, '--period=5.00', '--export'],
]

@pytest.mark.parametrize('command', TEST_CASES_2, ids=[' '.join(s) for s in TEST_CASES_2])
def test_valid_args(command: typing.List[str]) -> None:

    try:
        subprocess.check_output(command, stderr=subprocess.PIPE)
    except subprocess.CalledProcessError as e:
        pytest.fail(f'Subprocess failed. Output was: {e.stderr.decode()}')

    with open(PATH_RESULTS_FILE) as f:
        print(f.read())
