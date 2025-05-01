a = 0
b = 1
i = 0
limit = 100
while true
    a += b
    puts a, "\n"
    b += a
    puts b, "\n"
    i += 1
    if (i == limit)
        break
    end
end