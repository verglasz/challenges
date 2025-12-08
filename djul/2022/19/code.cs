using System;

class Program {
	private static void Main(string[] args)
	{
		int[] array = new int[10] { 0, 2, -3, -11, 17, -18, 17, -11, 13, -17 };
		int num = 112;
		int[] array2 = array;
		Console.WriteLine("The password is:");
		foreach (int num2 in array2) {
			num += num2;
			Console.Write((char)num);
		}
		Console.WriteLine("\nEnd of password");
	}
}

