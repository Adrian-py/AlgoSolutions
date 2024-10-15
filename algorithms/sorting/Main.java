public class Main {
    private static void print_array(int[] arr) {
        System.out.format("[");
        for (int i = 0; i < arr.length; i++) {
            System.out.format("%d", arr[i]);
            if (i < arr.length - 1) {
                System.out.format(", ");
            }
        }
        System.out.format("]\n");
    }

    public static void main(String[] args) {
        int[] input_array = new int[] {32, 18, 91, 167, 18, 57, 13, 97, 62};
        
        System.out.format("Before:\t");
        print_array(input_array);
        
        // Change this section to run specific algorithms
        // Sort MergeSort = new MergeSort();
        // MergeSort.sort(input_array);

        Sort QuickSort = new QuickSort();
        QuickSort.sort(input_array);
        
        System.out.format("After:\t");
        print_array(input_array);
    }
}
