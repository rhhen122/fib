local a, b, o
a, b, o = 0, 1, 0
while (true)
do
    print(a)
    o = a + b;
    a = b;
    b = o;
end