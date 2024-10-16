package sorting;

public class MergeSort implements Sort {
    public MergeSort() {}
    
    public void sort(int[] arr) {
        merge_sort(arr, 0, arr.length - 1);
    }

    private void merge_sort(int[] arr, int left, int right) {
        if (left < right) {
            int mid = left + (right - left) / 2;
            merge_sort(arr, left, mid);
            merge_sort(arr, mid + 1, right);
            merge(arr, left, right);
        }
    }

    private static void merge(int[] arr, int left, int right) {
        int mid = left + (right - left) / 2;
        int[] resultArray = new int[right - left + 1];
        int resPointer = 0;
        int leftPointer = left;
        int rightPointer = mid + 1;
        
        while (leftPointer < mid + 1 && rightPointer <= right) {
            if (arr[leftPointer] <= arr[rightPointer]) {
                resultArray[resPointer] = arr[leftPointer];
                leftPointer += 1;
            } else {
                resultArray[resPointer] = arr[rightPointer];
                rightPointer += 1;
            }
            resPointer += 1;
        } 
        
        while (leftPointer < mid + 1) {
            resultArray[resPointer] = arr[leftPointer];
            leftPointer += 1;
            resPointer += 1;
        }
        
        while (rightPointer <= right) {
            resultArray[resPointer] = arr[rightPointer];
            rightPointer += 1;
            resPointer += 1;
        }

        for (int i = 0; i < right - left + 1; i++) {
            arr[left + i] = resultArray[i];
        }
    }
} 