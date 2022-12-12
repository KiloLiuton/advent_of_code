with open('input', 'r') as f:
    data = [l.strip() for l in f.readlines()]

def get_trees_from_slope(vx, vy):
    trees_encountered = 0
    x = vx
    y = vy
    while y < len(data):
        if data[y][x] == '#':
            trees_encountered += 1
        x += vx
        y += vy
        if x >= len(data[0]):
            x -= len(data[0])
    return trees_encountered

print(get_trees_from_slope(3, 1))

# part 2
prod = 1
vx, vy = 1, 1
prod *= get_trees_from_slope(vx, vy)
vx, vy = 3, 1
prod *= get_trees_from_slope(vx, vy)
vx, vy = 5, 1
prod *= get_trees_from_slope(vx, vy)
vx, vy = 7, 1
prod *= get_trees_from_slope(vx, vy)
vx, vy = 1, 2
prod *= get_trees_from_slope(vx, vy)
print(prod)
