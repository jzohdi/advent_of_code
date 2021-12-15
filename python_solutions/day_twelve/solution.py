
edges: 'dict[str, set[str]]' = {}

def main():
    with open("./day_twelve/input.txt") as file:
        for line in file.readlines():
            s, e = line.strip().split("-")
            if s in edges:
                edges[s].add(e)
            else:
                edges[s] = set([e])
            if e in edges:
                edges[e].add(s)
            else:
                edges[e] = set([s])
    print(DFS('start', set(['start']), False))


def DFS(curr: str,visited: 'set[str]', has_doubled: bool) -> int:
    results = 0
    if curr == 'end':
        return 1

    if curr not in edges:
        return 0
    for neighbor in edges[curr]:
        if neighbor == 'start': # never visit start here
            continue
        if not is_small(neighbor): #if not is_small proceed as normal
            results += DFS(neighbor, visited, has_doubled)
        elif neighbor not in visited: # if is small and it is not in visited then add to visited
            results += DFS(neighbor, set([neighbor] + list(visited)), has_doubled)
        elif not has_doubled: # if neighbor is_small and is in visited and not has doubled
            results += DFS(neighbor, visited, True)

    return results

def is_small(node: str) -> bool:
    return not node.isupper()

if __name__ == '__main__':
    main()