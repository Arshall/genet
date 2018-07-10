#include "module.hpp"
#include "attr.hpp"
#include "exports.hpp"
#include "frame.hpp"
#include "layer.hpp"
#include "session.hpp"
#include "slice.hpp"
#include "token.hpp"
#include <cassert>

namespace genet_node {

namespace {
thread_local Module *globalInstance = nullptr;
}

Module::Module() {}

void Module::init(v8::Local<v8::Object> exports) {
  assert(!globalInstance);
  globalInstance = new Module();
  Token::init(exports);
  Slice::init(exports);
  SessionProfileWrapper::init(exports);
  SessionWrapper::init(exports);
  LayerWrapper::init(exports);
  FrameWrapper::init(exports);
  AttrWrapper::init(exports);

  v8::Isolate *isolate = v8::Isolate::GetCurrent();
  auto global = isolate->GetCurrentContext()->Global();
  v8::Local<v8::Value> args[1] = {exports};
  auto script =
      Nan::CompileScript(Nan::New(genet_embedded_js()).ToLocalChecked());
  auto func = Nan::RunScript(script.ToLocalChecked())
                  .ToLocalChecked()
                  .As<v8::Function>();
  func->Call(global, 1, args);
}

void Module::destroy() {
  assert(globalInstance);
  delete globalInstance;
  globalInstance = nullptr;
}

Module &Module::current() {
  assert(globalInstance);
  return *globalInstance;
}

Module::Class &Module::get(Slot slot) {
  if (classes.size() <= slot) {
    classes.resize(slot + 1);
  }
  return classes[slot];
}

} // namespace genet_node
