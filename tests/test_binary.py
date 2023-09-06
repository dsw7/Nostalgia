import json
import os
import subprocess
import typing
import pytest

PATH_BINARY = os.path.join(os.getcwd(), "target/debug/nos")
R1 = 99.5 * 1000
R2 = 21.5 * 1000

if not os.path.exists(PATH_BINARY):
    pytest.exit(f"{PATH_BINARY} does not exist!")

TEST_CASES_1 = [
    [PATH_BINARY],
    [PATH_BINARY, "--period=abc"],
    [PATH_BINARY, "-p=abc"],
    [PATH_BINARY, "--period=5", "--unit=foo"],
]


@pytest.mark.parametrize(
    "command", TEST_CASES_1, ids=[" ".join(s) for s in TEST_CASES_1]
)
def test_invalid_args(command: typing.List[str]) -> None:
    with pytest.raises(subprocess.CalledProcessError):
        subprocess.check_output(command, stderr=subprocess.DEVNULL)


def test_invalid_period() -> None:
    try:
        subprocess.check_output([PATH_BINARY, "--period=0"], stderr=subprocess.PIPE)
    except subprocess.CalledProcessError as e:
        assert (
            e.stderr.decode()
            == "Period is too small! Minimum period is 0.0001 seconds\n"
        )
    else:
        pytest.fail("Did not raise CalledProcessError")


def compute_expected_capacitance(period: float) -> typing.Dict[str, float]:
    f = 1 / period
    c = 1.44 / (f * (R1 + 2 * R2))
    return {
        "freq_in_hz": f,
        "cap_in_f": c,
        "cap_in_uf": c * 10**6,
        "cap_in_nf": c * 10**9,
    }


@pytest.mark.parametrize(
    "period", [0.01, 0.02, 0.1, 0.2, 1.0, 2.0, 10.0, 20.0, 100.0, 200.0]
)
def test_valid_args(period: float) -> None:
    try:
        output = subprocess.check_output(
            [PATH_BINARY, f"--period={period}", "--export"], stderr=subprocess.PIPE
        )
    except subprocess.CalledProcessError as e:
        pytest.fail(f"Subprocess failed. Output was: {e.stderr.decode()}")

    path_json = output.decode().split()[-1]
    with open(path_json) as f:
        results_nos = json.loads(f.read())

    results_exp = compute_expected_capacitance(period)

    assert pytest.approx(results_exp["freq_in_hz"]) == results_nos["freq_in_hz"]
    assert pytest.approx(results_exp["cap_in_f"]) == results_nos["cap_in_f"]
    assert pytest.approx(results_exp["cap_in_uf"]) == results_nos["cap_in_uf"]
    assert pytest.approx(results_exp["cap_in_nf"]) == results_nos["cap_in_nf"]
