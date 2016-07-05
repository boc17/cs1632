
public class Deliverable4 {

	public static void main(String[] args) {
		
		int[] array = new int[10];
		
		array[0] = 2;
		array[1] = 200;
		array[2] = 300;
		array[3] = 400;
        array[4] = 500;
        array[5] = 600;
        array[6] = 700;
        array[7] = 800;
        array[8] = 900;
        array[9] = 1000;
               
        int[] newArray = laboonify(array);
        
        for(int i = 0; i < newArray.length; i++)
        {
        	System.out.println(newArray[i]);
        }
		

	}
	
	public static int[] laboonify(int[] x)
	{
		int size = x.length+1;     //size of return array
		int[] arr = new int[size]; //make return array
		
		int totalSum = 0;             //sum
		
		for(int j = 0; j < (size-1); j++)
		{
			int input = squareNum(x[j]);   //square number
			arr[j] = input;                //put into new array
			totalSum = totalSum + input;   //add to total sum
			
		}
		
		arr[arr.length-1] = totalSum;      //add sum to array
		
		return arr;                        //return array
	}
	
	public static int squareNum(int x)
	{
		return x * x;            //double number
	}

}
