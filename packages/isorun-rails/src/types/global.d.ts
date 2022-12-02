export {};

declare global {
  type scalar = null | boolean | number | string;

  namespace Deno {
    namespace core {
      namespace ops {
        function op_app_send(action: string, args: scalar): Promise<scalar>;
      }
    }
  }
}
