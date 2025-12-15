using System;

namespace CSharpClient
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("--- C# Rust AllPassFilter Test ---");

            using (var apf = new AllPassFilter(1000, 10.5f, 0.5f))
            {
                Console.WriteLine("Processing single samples:");
                
                // インパルス応答のテスト
                for (int i = 0; i < 5; i++)
                {
                    float input = (i == 0) ? 1.0f : 0.0f;
                    float output = apf.Process(input);
                    Console.WriteLine($"[{i}] {output:F4}");
                }

                Console.WriteLine("\nProcessing block:");

                // ブロック処理のテスト
                float[] inputBlock = { 1.0f, 0.0f, 0.0f, 0.0f, 0.0f };
                float[] outputBlock = new float[5];

                apf.ProcessBlock(inputBlock, outputBlock);

                for (int i = 0; i < outputBlock.Length; i++)
                {
                    Console.WriteLine($"[{i}] {outputBlock[i]:F4}");
                }
            }

            Console.WriteLine("\nDone.");
        }
    }
}