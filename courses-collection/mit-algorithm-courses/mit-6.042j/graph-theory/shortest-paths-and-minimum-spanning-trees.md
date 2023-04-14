# 🌲 Shortest Paths and Minimum Spanning Trees

**Shortest Paths**

{% hint style="info" %}
The shortest path problem involves finding the shortest path between two vertices in a graph.
{% endhint %}

Dijkstra's algorithm

```python
import heapq

def dijkstra(graph, start):
    distances = {vertex: float('inf') for vertex in graph}
    distances[start] = 0
    heap = [(0, start)]
    
    while heap:
        (distance, current_vertex) = heapq.heappop(heap)
        if distance > distances[current_vertex]:
            continue
        for neighbor, weight in graph[current_vertex].items():
            path_cost = distance + weight
            if path_cost < distances[neighbor]:
                distances[neighbor] = path_cost
                heapq.heappush(heap, (path_cost, neighbor))
    
    return distances


# Example graph
graph = {
    'A': {'B': 3, 'C': 1},
    'B': {'A': 3, 'C': 4, 'D': 2},
    'C': {'A': 1, 'B': 4, 'D': 1},
    'D': {'B': 2, 'C': 1}
}

# Test case
assert dijkstra(graph, 'A') == {'A': 0, 'B': 3, 'C': 1, 'D': 2}

```

Bellman-Ford algorithm

```python
def bellman_ford(graph, start):
    distances = {vertex: float('inf') for vertex in graph}
    distances[start] = 0

    for _ in range(len(graph) - 1):
        for vertex in graph:
            for neighbor, weight in graph[vertex].items():
                path_cost = distances[vertex] + weight
                if path_cost < distances[neighbor]:
                    distances[neighbor] = path_cost

    return distances

# Example graph
graph = {
    'A': {'B': -1, 'C': 4},
    'B': {'C': 3, 'D': 2, 'E': 2},
    'C': {},
    'D': {'B': 1, 'C': 5},
    'E': {'D': -3}
}

# Test case
assert bellman_ford(graph, 'A') == {'A': 0, 'B': -1, 'C': 2, 'D': -2, 'E': 1}

```

**Minimum Spanning Trees**

{% hint style="info" %}
The minimum spanning tree problem involves finding a spanning tree of a graph with the smallest possible weight
{% endhint %}

Prim's algorithm

```python
import heapq

def prim(graph, start):
    visited = set()
    edges = [(0, start)]
    min_spanning_tree = []
    
    while edges:
        weight, vertex = heapq.heappop(edges)
        if vertex not in visited:
            visited.add(vertex)
            min_spanning_tree.append((weight, vertex))
            for neighbor, edge_weight in graph[vertex].items():
                if neighbor not in visited:
                    heapq.heappush(edges, (edge_weight, neighbor))
    
    return min_spanning_tree

# Example graph
graph = {
    'A': {'B': 2, 'C': 3},
    'B': {'A': 2, 'C': 1, 'D': 1},
    'C': {'A': 3, 'B': 1, 'D': 2},
    'D': {'B': 1, 'C': 2}
}

# Test case
assert prim(graph, 'A') == [(2, 'B'), (1, 'C'), (1, 'D')]

```

Kruskal's algorithm

```python
def kruskal(graph):
    parents = {}
    rank = {}
    for vertex in graph:
        parents[vertex] = vertex
        rank[vertex] = 0

    def find(vertex):
        if parents[vertex] != vertex:
            parents[vertex] = find(parents[vertex])
        return parents[vertex]

    def union(vertex1, vertex2):
        root1 = find(vertex1)
        root2 = find(vertex2)
        if root1 != root2:
            if rank[root1] > rank[root2]:
                parents[root2] = root1
            else:
                parents[root1] = root2
                if rank[root1] == rank[root2]:
                    rank[root2] += 1

    min_spanning_tree = []
    edges = [(weight, vertex1, vertex2) for vertex1 in graph for vertex2, weight in graph[vertex1].items()]
    edges.sort()
    for weight, vertex1, vertex2 in edges:
        if find(vertex1) != find(vertex2):
            union(vertex1, vertex2)
            min_spanning_tree.append((weight, vertex1, vertex2))

    return min_spanning_tree

# Example graph
graph = {
    'A': {'B': 7, 'D': 5},
    'B': {'A': 7, 'C': 8, 'D': 9, 'E': 7},
    'C': {'B': 8, 'E': 5},
    'D': {'A': 5, 'B': 9, 'E': 15, 'F': 6},
    'E': {'B': 7, 'C': 5, 'D': 15, 'F': 8, 'G': 9},
    'F': {'D': 6, 'E': 8, 'G': 11},
    'G': {'E': 9, 'F': 11}
}

# Test case
assert kruskal(graph) == [(5, 'A', 'D'), (7, 'B', 'E'), (5, 'C', 'E'), (6, 'D', 'F'), (7, 'E', 'C'), (9, 'E', 'G')]
```
