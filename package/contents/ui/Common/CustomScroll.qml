import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import org.kde.plasma.extras as PlasmaExtras
import org.kde.plasma.core as PlasmaCore
import org.kde.plasma.components as PlasmaComponents
import org.kde.kirigami as Kirigami

PlasmaComponents.ScrollView {
    Layout.fillWidth: true
    Layout.fillHeight: true
    property alias child: packageView.delegate
    property bool searchMode: false
    property alias fModel: packageView.model
    contentItem: ListView {
        id: packageView
        spacing: Kirigami.Units.smallSpacing
        currentIndex: -1
        highlight: PlasmaExtras.Highlight { }
        Kirigami.PlaceholderMessage {
            anchors.centerIn: parent
            width: parent.width - Kirigami.Units.largeSpacing * 4
            visible: parent.count === 0 && !isBusy
            text: {
                if(searchMode)return "No results."
                else if(error !== "") return "Some error occurred."
                else return "You are up to date!"
            }
            icon.name: {
                if(searchMode) return "edit-none"
                else if(error !== "") return "data-error"
                else return "checkmark"
            }
        }
    }
}
