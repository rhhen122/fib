local a, b
a, b = 0, 1
while (true)
do
    a = a + b
    print(a)
    b = b + a
    print(b)
end
