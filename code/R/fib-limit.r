a <- 0
b <- 1
i <- 0
limit <- 2
while (TRUE) {
    a <- a + b
    print(a)
    i <- i + 1
    b <- b + a
    print(b)
    i <- i + 1
    if (i == limit) {
        break
    }
}
