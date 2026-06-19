const std = @import("std");
const net = std.Io.net;

pub const Asset = struct {
    path: []const u8,
    data: []const u8,
};

pub fn serve_assets(
    allocator: *std.mem.Allocator,
    io: std.Io,
    port: u16,
    assets: []Asset,
) !void {
    const address = net.IpAddress.parse("0.0.0.0", port) catch |err| {
        std.log.err("Failed to parse address: {}\n", .{err});
        return err;
    };

    var listener = address.listen(io, .{}) catch |err| {
        std.log.err("Failed to start TCP listener: {}\n", .{err});
        return err;
    };
    defer listener.deinit();

    std.log.info("Binfuse HTTP server started on :{d}\n", .{port});

    while (true) {
        const conn = listener.accept(io) catch |err| {
            std.log.err("Failed to accept connection: {}\n", .{err});
            continue;
        };
        defer conn.close(io);

        const mut_assets = assets;

        const result = handle_request(allocator, io, conn, mut_assets) catch |err| {
            std.log.err("Error handling request: {}\n", .{err});
            continue;
        };
        _ = result;
    }
}

fn handle_request(
    allocator: *std.mem.Allocator,
    io: std.Io,
    conn: net.Stream,
    assets: []Asset,
) !void {
    _ = allocator;
    var reader = conn.reader(io, &[_]u8{});
    var writer = conn.writer(io, &[_]u8{});

    var buffer: [4096]u8 = undefined;
    const bytes_read = try reader.interface.readSliceShort(&buffer);
    const request = buffer[0..bytes_read];

    const path_start = std.mem.indexOf(u8, request, " ").? + 1;
    const path_end = std.mem.indexOf(u8, request[path_start..], " ").?;
    const path = request[path_start .. path_start + path_end];

    var asset_data: ?[]const u8 = null;
    for (assets) |asset| {
        if (std.mem.eql(u8, asset.path, path)) {
            asset_data = asset.data;
            break;
        }
    }

    var header_buffer: [1024]u8 = undefined;

    if (asset_data) |data| {
        const header = try std.fmt.bufPrint(&header_buffer, "HTTP/1.1 200 OK\r\nContent-Length: {d}\r\nContent-Type: text/html\r\n\r\n", .{data.len});
        try writer.interface.writeAll(header);

        try writer.interface.writeAll(data);
    } else {
        const header = try std.fmt.bufPrint(&header_buffer, "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n", .{});
        try writer.interface.writeAll(header);
    }
}
