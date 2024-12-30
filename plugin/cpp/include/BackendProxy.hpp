#pragma once

#include "cxxbridge/backend.rs.hpp"
#include <QObject>
#include <qqmlintegration.h>
#include <qtmetamacros.h>
#include <qstring.h>

class BackendProxy : public QObject {
private:
  Q_OBJECT
  Q_PROPERTY(QString getMsg READ getMsg NOTIFY msgChanged)

public:
  explicit BackendProxy(QObject *parent = nullptr);
  ~BackendProxy() = default;

  QString getMsg() const;
//  bool isBusy(bool inp) const;
//  bool isError(bool inp) const;

Q_SIGNALS:
  void msgChanged(QString const &inp);
  void isBusyChanged(bool const &inp);
  void isErrorChanged(bool const &inp);

private:
    rust::Box<rust::Backend> _backend;
};
