input = """
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"""
input = """
237822270988608, 164539183264530, 381578606559948 @ 115, 346, -342
287838354624648, 284335343503076, 181128681512377 @ -5, -84, 175
341046208911993, 120694764237967, 376069872241870 @ -74, 129, -78
275834119712623, 395388307575057, 177270820376760 @ 90, -111, -10
284284433233698, 358506322947508, 169341917878543 @ 20, 133, 71
"""

def parse_line(s: str):
    # return [list(map(int, part.split(','))) for part in s.replace('\n', '').split('@')]
    return [list(map(int, part.split(','))) for part in s.strip().split('@') if part.strip()]
v = [parse_line(s) for s in input.splitlines() if parse_line(s)]
print(v)

def solve_for_n():
    print("Hi")
    num_vars = len(v)
    var_names = ['x', 'y', 'z'] + ['t{}'.format(i) for i in range(1, num_vars + 1)]
    sp = ['sp{}'.format(i) for i in range(1, 4)]
    var(var_names)
    var(sp)

    equations = []
    for i in range(num_vars):
        print(f"Constructing equations for hailstone {i}")
        t_var = var(var_names[3+i])
        # eq = [v[i][0][j] + t_var * v[i][1][j] == sp[j] + t_var * var(var_names[j]) for j in range(3)]
        eq = [v[i][0][j] + t_var * v[i][1][j] == var(sp[j]) + t_var * var(var_names[j]) for j in range(3)]
        equations.extend(eq)

    print("Constructed system of equations, beginning solve")
    solutions = solve(equations, sp1, sp2, sp3, *var(var_names), to_poly_solve=True)
    print("Solve complete")
    sp1_value = solutions[0][0]
    sp2_value = solutions[0][1]
    sp3_value = solutions[0][2]
    print(sp1_value + sp2_value + sp3_value)
    return solutions

solve_for_n()
