def rotate(guard):
    (position, (dx, dy)) = guard;
    return (position, (-dy, dx))


def patrol(grid, guard):
    position, orientation = guard
    visited = {position}

    while True:
        x, y = position
        dx, dy = orientation
        xn, yn = x + dx, y + dy

        if xn < 0 or yn < 0 or yn >= len(grid) or xn >= len(grid[yn]):
            break

        if grid[yn][xn] == '#':
            position, orientation = rotate((position, orientation))
            continue

        visited.add((xn, yn))
        position = xn, yn

    return len(visited)


def main():
    grid = list(map(list, open('input.txt').read().splitlines()))
    (x, y) = next((x, y) for (y, row) in enumerate(grid)
                         for (x, c) in enumerate(row)
                         if c not in "#.")

    directions = {'^': (0, -1), '>': (1, 0), 'v': (0, 1), '<': (-1, 0)}
    dx, dy = directions[grid[y][x]]
    guard = ((x, y), (dx, dy))

    print(patrol(grid, guard))

main()
