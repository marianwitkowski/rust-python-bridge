import rust_python_bridge

LOOPS = 1_000_000
print(rust_python_bridge.approximate_pi(LOOPS))
print(rust_python_bridge.approximate_pi_monte_carlo(LOOPS))