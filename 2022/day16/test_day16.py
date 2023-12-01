from day16 import get_test_lines, parse, open_valves

def test_rate_of_release():
    lines = get_test_lines()
    G = parse(lines)
    for args, expected in [
            # Stay in-place
            ((G, ("CC"), 2), (2, frozenset(["CC"]))),
            ((G, ("CC"), 1), (0, frozenset())),
            ((G, ("CC"), 0), (0, frozenset())),
            # Movement
            ((G, ("EE"), 3), (20, frozenset(["DD"]))),
            ((G, ("EE"), 4), (40, frozenset(["DD"]))),
            ((G, ("EE"), 5), (63, frozenset(["DD", "EE"]))),
            ]:
        assert open_valves(*args) == expected, args

def test_dual_rate_of_release():
    lines = get_test_lines()
    G = parse(lines)
    for args, expected in [
            # Stay in-place
            ((G, ("CC", "FF"), 2), (2, frozenset(["CC"]))),
            ((G, ("CC", "EE"), 2), (5, frozenset(["CC", "EE"]))),
            ((G, ("CC", "EE"), 1), (0, frozenset())),
            ((G, ("CC", "EE"), 0), (0, frozenset())),
            # Movement
            ((G, ("CC", "FF"), 3), (23, frozenset(["DD", "EE"]))),
            ((G, ("CC", "FF"), 4), (22+(20*2), frozenset(["DD", "HH"]))),
            ((G, ("CC", "FF"), 5), ((20*3 + 3)+(22*2), frozenset(["DD", "EE", "HH"]))),
            ]:
        assert open_valves(*args) == expected, args

