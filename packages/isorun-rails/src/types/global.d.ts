export {};

declare global {
  type scalar = null | boolean | number | string;
  type serde = scalar | Record<string, serde> | Array<serde>;

  namespace Deno {
    namespace core {
      namespace ops {
        function op_app_send(action: string, args: serde): Promise<serde>;
      }
    }
  }
}
