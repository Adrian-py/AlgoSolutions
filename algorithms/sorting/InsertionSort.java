package sorting;

public class InsertionSort implements Sort {
    public void sort(int[] arr) {
        this.insertion_sort(arr);
    }

    private void insertion_sort(int[] arr) {
        for (int i = 1; i < arr.length; i++) {
            int current_el = arr[i];
            int j = i - 1;

            while (j >= 0 && arr[j] > current_el) {
                arr[j + 1] = arr[j];
                j = j - 1;
            }

            arr[j + 1] = current_el;
        }
    }
}
