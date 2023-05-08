// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.
//
// This is a GENERATED FILE, changes made here WILL BE LOST.
//

import Foundation

public final class FirstStruct {
    let rawValue: OpaquePointer

    init(rawValue: OpaquePointer) {
        self.rawValue = rawValue
    }

    public init?(string: String?) {
        guard let result = FirstStructCreate(string) else {
            return nil
        }

        self.rawValue = result
    }

    deinit {
        FirstStructDelete(self.rawValue)
    }

    public static func withOptionalInt(first_param: Int32?) -> Bool? {
        guard let result = FirstStructWithOptionalInt(first_param) else {
            return nil
        }
        return result
    }

    public static func withOptionalStruct(first_param: SomeStruct?) -> Bool? {
        let first_param = first_param?.rawValue
        guard let result = FirstStructWithOptionalStruct(first_param) else {
            return nil
        }
        return result
    }

    public static func withOptionalString(first_param: String?) -> Bool? {
        let ptr: UnsafeRawPointer?
        if let first_param = first_param {
            ptr = TWStringCreateWithNSString(first_param)
        } else {
            ptr = nil
        }
        defer {
            if let first_param  = ptr {
                TWStringDelete(first_param)
            }
        }
        let first_param = ptr

        guard let result = FirstStructWithOptionalString(first_param) else {
            return nil
        }
        return result
    }

    public static func withOptionalEnum(first_param: SomeEnum?) -> Bool? {
        let first_param = SomeEnum(rawValue: first_param.rawValue)
        guard let result = FirstStructWithOptionalEnum(first_param) else {
            return nil
        }
        return result
    }

    public var withOptionalInt: Int32? {
        let obj = self.rawValue
        guard let result = FirstStructWithOptionalInt(obj) else {
            return nil
        }
        return result
    }

    public var withOptionalString: String? {
        let obj = self.rawValue
        guard let result = FirstStructWithOptionalString(obj) else {
            return nil
        }
        return TWStringNSString(result)
    }

    public var withOptionalStruct: SomeStruct? {
        let obj = self.rawValue
        guard let result = FirstStructWithOptionalStruct(obj) else {
            return nil
        }
        return SomeStruct(rawValue: result)
    }

    public var withOptionalEnum: SomeEnum? {
        let obj = self.rawValue
        guard let result = FirstStructWithOptionalEnum(obj) else {
            return nil
        }
        return SomeEnum(rawValue: result.rawValue)!
    }
}
