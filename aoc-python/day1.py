FILE = "../inputs/day1"

with open(FILE) as f:
    nums = [int(line.strip()) for line in f.readlines()]

# problem 1:
sol_1 = sum(a < b for (a, b) in zip(nums, nums[1:]))

# problem 2:
nums_2 = [sum(n) for n in zip(nums, nums[1:], nums[2:])]
sol_2 = sum(a < b for (a, b) in zip(nums_2, nums_2[1:]))
