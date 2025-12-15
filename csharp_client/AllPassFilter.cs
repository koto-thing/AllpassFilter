using System;
using System.Runtime.InteropServices;

namespace CSharpClient
{
    public class AllPassFilter : IDisposable
    {
        /// <summary>
        /// ネイティブメソッドのインポート
        /// </summary>
        private static class NativeMethods
        {
            const string DllName = "allpass_filter";

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern IntPtr allpass_create(UIntPtr max_delay, float initial_delay, float gain);

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern void allpass_destroy(IntPtr ptr);

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern float allpass_process(IntPtr ptr, float input);

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern void allpass_process_block(IntPtr ptr, float[] input, float[] output, UIntPtr len);

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern void allpass_set_delay(IntPtr ptr, float delay);

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern void allpass_set_gain(IntPtr ptr, float gain);

            [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
            public static extern void allpass_set_smoothing(IntPtr ptr, float factor);
        }

        private IntPtr _handle;         // ネイティブオブジェクトのハンドル
        private bool _disposed = false; // 破棄フラグ

        /// <summary>
        /// コンストラクタ
        /// </summary>
        public AllPassFilter(int max_delay, float initial_delay, float gain)
        {
            _handle = NativeMethods.allpass_create((UIntPtr)max_delay, initial_delay, gain);
        }

        /// <summary>
        /// デストラクタ
        /// </summary>
        ~AllPassFilter()
        {
            Dispose(false);
        }

        /// <summary>
        /// メモリ開放
        /// </summary>
        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }

        protected virtual void Dispose(bool disposing)
        {
            if (!_disposed)
            {
                if (_handle != IntPtr.Zero)
                {
                    NativeMethods.allpass_destroy(_handle);
                    _handle = IntPtr.Zero;
                }

                _disposed = true;
            }
        }

        public float Process(float input)
        {
            CheckDisposed();
            return NativeMethods.allpass_process(_handle, input);
        }

        public void ProcessBlock(float[] input, float[] output)
        {
            CheckDisposed();
            if (output.Length < input.Length)
            {
                Array.Resize(ref output, input.Length);
            }

            NativeMethods.allpass_process_block(_handle, input, output, (UIntPtr)input.Length);
        }

        public void SetDelay(float delay)
        {
            CheckDisposed();
            NativeMethods.allpass_set_delay(_handle, delay);
        }

        public void SetGain(float gain)
        {
            CheckDisposed();
            NativeMethods.allpass_set_gain(_handle, gain);
        }

        public void SetSmoothing(float factor)
        {
            CheckDisposed();
            NativeMethods.allpass_set_smoothing(_handle, factor);
        }

        private void CheckDisposed()
        {
            if (_disposed) throw new ObjectDisposedException(nameof(AllPassFilter));
        }
    }
}