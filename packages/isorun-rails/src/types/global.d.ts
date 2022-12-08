export {};

declare global {
  type scalar = null | boolean | number | string;
  type serde = scalar | Record<string, serde> | Array<serde>;

  namespace Deno {
    namespace core {
      namespace ops {
        function op_send_to_ruby(message: serde): Promise<serde>;
      }
    }
  }
}
