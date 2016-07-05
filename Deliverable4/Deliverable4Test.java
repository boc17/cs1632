import static org.junit.Assert.*;
import java.util.Random;

import org.junit.Test;

public class Deliverable4Test {

	
	//Test that the array being return is always 1 size bigger than the
	//input given to laboonify
	@Test
	public void testSize() {
		
		Random r = new Random();
		
		for(int j = 0; j < 100; j++)
		{
			int size = r.nextInt(100) + 1;		     //random number between 1-100
			int[] input = new int[size];
			
			for(int i = 0; i < input.length; i++)
			{
				input[i] = r.nextInt(100)+1;        //insert element to array
			}
			
			int x = Deliverable4.laboonify(input).length;   //get length of laboonify
			
			assertEquals(x, size+1);    //check if sizes are equal
		}
	}
	
	//Test that the last input in the array is the correct sum
	//of all the elements in the array
	@Test
	public void testSum() {
		
		Random r = new Random();  //random number generator
		
		for(int j = 0; j < 100; j++)
		{
			int size = r.nextInt(100) + 1;		 //random number between 1-100	
			int[] input = new int[size];
			
			int total = 0;
			
			for(int i = 0; i < input.length; i++)
			{
				input[i] = r.nextInt(100)+1;             //insert element to array
				
				total = total + (input[i] * input[i]);    //add double to total
			}
			
			int[] x = Deliverable4.laboonify(input);
			
			assertEquals(x[x.length-1], total);
		}
	}
	
	//Test that every element is the square of the input excluding the last
	//element
	@Test
	public void testSquare() {
		
		Random r = new Random();
		
		for(int j = 0; j < 100; j++)
		{			
			int size = r.nextInt(100) + 1;	     //random number between 1-100
			int[] input = new int[size];
			int[] inputSquare = new int[size];
			
			for(int i = 0; i < input.length; i++)
			{
				input[i] = r.nextInt(100)+1;             //regular input
				inputSquare[i] = input[i] * input[i];    //square input
			}
			
			int[] x = Deliverable4.laboonify(input);
			
			for(int w = 0; w < x.length-1; w ++)
			{
				assertEquals(x[w], inputSquare[w]);    //test all the doubles
			}
		}
	}

}
