def count_valid_triangles(triangles):
    num_valid_triangles = 0
    for triangle in triangles:
        num_valid_triangles += (triangle[0] < triangle[1] + triangle[2]) and \
                               (triangle[1] < triangle[0] + triangle[2]) and \
                               (triangle[2] < triangle[0] + triangle[1])
    return num_valid_triangles

def trip_tris_generator(triangles):
    tri_iter = iter(triangles)
    while True:
        yield (tri_iter.next(), tri_iter.next(), tri_iter.next())

def main():
    with open('input.txt', 'r') as fp:
        triangles = [[int(x) for x in line.split()] for line in fp]

    assert len(triangles) % 3 == 0
    part2_triangles = [tri for trip_tris in trip_tris_generator(triangles) for tri in zip(*trip_tris)]

    print "Part 1: {}".format(count_valid_triangles(triangles))
    print "Part 2: {}".format(count_valid_triangles(part2_triangles))

if __name__ == "__main__":
    main()
