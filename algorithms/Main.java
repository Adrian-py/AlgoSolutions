import searching.*;
import sorting.*;

public class Main {
    public static void main(String[] args) {
        int[] input_array = new int[] {32, 18, 91, 167, 18, 57, 13, 97, 62};

        Sort MergeSort = new MergeSort();
        MergeSort.sort(input_array);

        int target = input_array[4];
        Search BinarySearch = new BinarySearch();
        int binary_search_res = BinarySearch.search(input_array, target);
        System.out.println("Element found at index: " + binary_search_res);
        
        Search LinearSearch = new BinarySearch();
        int linear_search_res = LinearSearch.search(input_array, target);
        System.out.println("Element found at index: " + linear_search_res);
    }
}
