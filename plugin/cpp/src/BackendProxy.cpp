#include "BackendProxy.hpp"

BackendProxy::BackendProxy(QObject *parent)
    : QObject{parent}, _backend{rust::make_backend()} {}

//void BackendProxy::isBusy(bool inp) const {
//    if (_backend->is_busy() != inp) {
//        _backend->set_busy()
//        Q_EMIT isBusyChanged(_backend->isBusy());
//    }
//}
//void BackendProxy::isError(bool inp) const {
//    if (_backend->is_error() != inp) {
//        Q_EMIT isErrorChanged(_backend->is_error());
//    }
//}

//void BackendProxy::msg() const {
//    Q_EMIT msgChanged(_backend->get_msg());
//}
QString BackendProxy::getMsg() const {
    rust::cxxbridge1::String rustStr = _backend->msg();
    std::string stdStr(rustStr.begin(), rustStr.end());
    return QString::fromStdString(stdStr);
}
#include "moc_BackendProxy.cpp"
