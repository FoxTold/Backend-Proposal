#include "crow.h"
#include <vector>
#include <string>

// Define the Album structure
struct Album {
    std::string id;
    std::string title;
    std::string artist;
    double price;
};

// Initialize the albums vector with some sample data
std::vector<Album> albums = {
    {"1", "Blue Train", "John Coltrane", 56.99},
    {"2", "Jeru", "Gerry Mulligan", 17.99},
    {"3", "Sarah Vaughan and Clifford Brown", "Sarah Vaughan", 39.99}
};

// Main function
int main() {
    crow::SimpleApp app;

    // Define an endpoint that returns the list of albums
    CROW_ROUTE(app, "/albums")
    ([]() {
        crow::json::wvalue jsonAlbums;
        std::vector<crow::json::wvalue> albumList;

        // Iterate over albums and create JSON for each album
        for (const auto& album : albums) {
            crow::json::wvalue albumJson;
            albumJson["id"] = album.id;
            albumJson["title"] = album.title;
            albumJson["artist"] = album.artist;
            albumJson["price"] = album.price;

            // Add the album JSON object to the albumList vector
            albumList.push_back(std::move(albumJson));
        }

        // Assign the albumList to the "albums" field of the response
        jsonAlbums["albums"] = std::move(albumList);

        // Return the response
        return jsonAlbums;
    });

    // Start the server on port 18080
    app.port(18080).multithreaded().run();
}