import { useEffect, useRef, useState } from 'react';

interface UseResizeObserverOptions {
  onResize?: (size: { x: number; y: number; width: number; height: number }) => void;
}

export const useResizeObserver = <T extends HTMLElement>(
  options?: UseResizeObserverOptions,
) => {
  const [size, setSize] = useState<{ width?: number; height?: number }>({});
  const ref = useRef<T>(null);
  const lastRectRef = useRef('');

  const onResizeRef = useRef(options?.onResize);
  onResizeRef.current = options?.onResize;

  useEffect(() => {
    if (!ref.current) return;

    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (entry) {
        const { x, y, width, height } = entry.target.getBoundingClientRect();

        const key = [x, y, width, height]
          .map((v) => Math.round(v * 100) / 100).join(',');
        if (key === lastRectRef.current) return;
        lastRectRef.current = key;
        setSize((previous) =>
          previous.width === width && previous.height === height
            ? previous : { width, height });

        onResizeRef.current?.({ x, y, width, height });
      }
    });

    observer.observe(ref.current);

    return () => {
      observer.disconnect();
    };
  }, []);

  return { ref, width: size.width, height: size.height };
};
