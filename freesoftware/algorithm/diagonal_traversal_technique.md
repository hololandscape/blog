# Diagonal Traversal Technique

Or zig-zag traversal technique

This technique is involves iterating over the matrix in a zigzag pattern, starting from the top left corner, and moving towards the bottom right corner.

## Zigzag pattern

Zigzag pattern is a pattern in which the elements are traversed diagonally.

## Example of implementation

```Python
# 1 2 3
# 4 5 6
# 7 8 9

# result:1 2 4 7 5 3 6 8 9

def diagonal_traversal(matrix):
    result = []
    row = 0
    col = 0
    row_len = len(matrix)
    col_len = len(matrix[0])
    for i in range(row_len * col_len):
        result.append(matrix[row][col])
        if (row + col) % 2 == 0:
            if col == col_len - 1:
                row += 1
            elif row == 0:
                col += 1
            else:
                row -= 1
                col += 1
        else:
            if row == row_len - 1:
                col += 1
            elif col == 0:
                row += 1
            else:
                row += 1
                col -= 1
    return result

```







## Questions

[Diagonal Traverse](https://leetcode.com/problems/diagonal-traverse/)