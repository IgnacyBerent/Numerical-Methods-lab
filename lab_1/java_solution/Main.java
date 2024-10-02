import java.util.concurrent.TimeUnit;

public class Main {
    public static double[] solution() {
        double sol = 0.0;
        double k = 1.0;
        double check = 0.0;
        while (true) {
            double partialSol = 1 / Math.pow(k, 2);
            sol += partialSol;
            if (sol == check) {
                return new double[]{sol, k};
            }
            check = sol;
            k += 1;
        }
    }

    public static void main(String[] args) {
        long startTime = System.nanoTime();
        double[] result = solution();
        long endTime = System.nanoTime();
        double approx = result[0];
        double k = result[1];
        System.out.println("Java Solution:");
        System.out.println("final approx: " + approx);
        System.out.println("n_max: " + k);
        System.out.println("time: " + TimeUnit.NANOSECONDS.toMillis(endTime - startTime) + " ms");
        double percentError = Math.abs(approx - Math.pow(Math.PI, 2) / 6) / (Math.pow(Math.PI, 2) / 6) * 100;
        System.out.println("percent error: " + percentError);
    }
}