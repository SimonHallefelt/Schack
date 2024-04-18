# take data from new_raw_data and add it to raw_data and then delete new_raw_data
# raw_data dose not permit to have duplicate data

print("hello1")
new_raw_data = open("data\\new_raw_data.txt", "r")
lines = new_raw_data.readlines()

raw_data = open("data\\raw_data.txt", "r")
lines2 = raw_data.readlines()


print("hello2")
data = set()
for line in lines:
    data.add(line)

for line in lines2:
    data.add(line)


print("hello3")
raw_data = open("data\\raw_data.txt", "w")
for line in data:
    raw_data.write(line)


print("Done!")