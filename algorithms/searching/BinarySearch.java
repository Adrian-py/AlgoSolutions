package searching;

public class BinarySearch implements Search {
    public BinarySearch() {}

    public int search(int[] arr, int target) {
        return this.binary_search(arr, target);
    }

    private int binary_search(int[] arr, int target) {
        int left = 0;
        int right = arr.length - 1;
        
        while (left <= right) {
            int mid = left + (right - left) / 2;

            if (arr[mid] == target) {
                return mid;
            } else if (arr[mid] > target) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return -1;
    }
}
