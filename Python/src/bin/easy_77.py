# Daily Coding Problem: 77
# Difficulty: Easy
# Asked by: Snapchat
# Author: TenthEdict

def merge_intervals(intervals):
    intervals = sorted(intervals)

    if not intervals:
        return []

    result = [intervals[0]]

    for interval in intervals[1:]:
        a,b = result[-1]
        c,d = interval
        if c <= b:
            result[-1] = (a, max(b,d))
        else:
            result.append(interval)
    
    return result

print(merge_intervals([(1, 3), (5, 8), (4, 10), (20, 25)]))  # [(1, 3), (4, 10), (20, 25)]
print(merge_intervals([]))                                    # []
print(merge_intervals([(5, 10)]))                             # [(5, 10)]
print(merge_intervals([(1, 10), (2, 5), (3, 7)]))            # [(1, 10)] - all merge
