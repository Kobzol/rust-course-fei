items = [1, 2, 4, 8, 3]

for item in items:
    if item % 2 == 0:
        items.remove(item)

print(items)
