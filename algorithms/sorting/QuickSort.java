package sorting;

public class QuickSort implements Sort {
    public void sort(int[] arr) {
        this.quick_sort(arr, 0, arr.length - 1);
    }

    private void swap(int[] arr, int first_index, int second_index) {
        int temp = arr[first_index];
        arr[first_index] = arr[second_index];
        arr[second_index] = temp;
    }

    private int partition(int[] arr, int left, int right) {
        int pivot = arr[right];
        int i = left - 1;

        for (int j = left; j < right; j++) {
            if (arr[j] <= pivot) {
                i += 1;
                this.swap(arr, i, j);
            }
        }
        i += 1;
        this.swap(arr, i, right);
        return i;
    }

    private void quick_sort(int[] arr, int left, int right) {
        if (left < right) {
            int last_pivot = partition(arr, left, right);
            quick_sort(arr, left, last_pivot - 1);
            quick_sort(arr, last_pivot + 1, right);
        }
    }
}
