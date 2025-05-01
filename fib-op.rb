a = 0
b = 1
def cus_puts(val)
    puts "#{val}\n"
end
while true
    a += b
    cus_puts(a)
    b += a
    cus_puts(b)
end