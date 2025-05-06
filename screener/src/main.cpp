#include <gio/gio.h>
#include <iostream>
#include <fstream>

bool takeScreenshot(const std::string &outputFile) {
    GError *error = nullptr;

    // Connect to the org.gnome.Shell.Screenshot D-Bus interface
    GDBusConnection *connection = g_bus_get_sync(G_BUS_TYPE_SESSION, nullptr, &error);
    if (!connection) {
        std::cerr << "Failed to connect to session bus: " << error->message << std::endl;
        g_error_free(error);
        return false;
    }

    // Call the Screenshot method
    GVariant *result = g_dbus_connection_call_sync(
        connection,
        "org.gnome.Shell.Screenshot",    // Bus name
        "/org/gnome/Shell/Screenshot",  // Object path
        "org.gnome.Shell.Screenshot",   // Interface name
        "Screenshot",                   // Method name
        g_variant_new("(bss)", false, nullptr, false), // Arguments: interactive=false, filename=nullptr, flash=false
        G_VARIANT_TYPE("(bs)"),         // Return type: (success: boolean, filename: string)
        G_DBUS_CALL_FLAGS_NONE,
        -1,
        nullptr,
        &error
    );

    if (!result) {
        std::cerr << "D-Bus call failed: " << error->message << std::endl;
        g_error_free(error);
        g_object_unref(connection);
        return false;
    }

    // Parse the result
    gboolean success;
    const char *tempFile;
    g_variant_get(result, "(bs)", &success, &tempFile);

    if (!success) {
        std::cerr << "Screenshot capture failed." << std::endl;
        g_variant_unref(result);
        g_object_unref(connection);
        return false;
    }

    // Copy the temporary file to the desired location
    std::ifstream src(tempFile, std::ios::binary);
    std::ofstream dest(outputFile, std::ios::binary);
    dest << src.rdbuf();

    std::cout << "Screenshot saved to: " << outputFile << std::endl;

    g_variant_unref(result);
    g_object_unref(connection);
    return true;
}

int main() {
    std::string outputFile = "/tmp/screenshot.png";
    if (!takeScreenshot(outputFile)) {
        std::cerr << "Failed to take screenshot." << std::endl;
        return 1;
    }
    return 0;
}
