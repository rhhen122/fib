a = 0
b = 1
i = 0
limit = 2
while true
    a += b
    puts a, "\n"
    i += 1
    b += a
    puts b, "\n"
    i += 1
    if (i == limit)
        break
    end
end