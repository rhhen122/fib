public class fibop {
    public static void main(String[] args) {
        int a = 0;
        int b = 1;
        while (true) {
            a += b;
            System.out.println(a);
            b += a;
            System.out.println(b);
        }
    }
}